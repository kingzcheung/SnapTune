use anyhow::{bail};
use opencv::{
    core::{self, MatExprTraitConst, MatTrait, MatTraitConst},
    dnn::{self, NetTrait, NetTraitConst, DNN_BACKEND_OPENCV, DNN_TARGET_CPU}, types::{VectorOfu8, VectorOfi32},
};
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;
#[derive(Serialize, Deserialize, Debug)]
pub struct BoxDetection {
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
    pub class: i32,
    pub conf: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Detections {
    pub detections: Vec<BoxDetection>,
}
/// Contains information about original input image and effective size fed into the model.
pub struct MatInfo {
    width: f32,
    height: f32,
    scaled_size: f32,
}
pub struct Yolov5 {
    model: dnn::Net,
}
impl Yolov5 {
    pub fn new(onnx_file: String) -> Result<Self, anyhow::Error> {
        let mut net = opencv::dnn::read_net_from_onnx(onnx_file.as_str())?;
        net.set_preferable_backend(DNN_BACKEND_OPENCV as i32)?;
        net.set_preferable_target(DNN_TARGET_CPU as i32)?;
        Ok(Self { model: net })
    }
    pub fn detect(&mut self, img: &core::Mat, conf_thresh: f32, nms_thresh: f32) -> opencv::Result<Detections,anyhow::Error> {
        detect(&mut self.model, img, conf_thresh, nms_thresh)
    }
}
/// Run detection on input image.
pub fn detect(
    model: &mut dnn::Net,
    img: &core::Mat,
    conf_thresh: f32,
    nms_thresh: f32,
) -> opencv::Result<Detections,anyhow::Error> {
    let mat_info = MatInfo {
        width: img.cols() as f32,
        height: img.rows() as f32,
        scaled_size: 640f32,
    };
    let padded_mat = prepare_input(img)?;
    // dnn blob
    let blob = opencv::dnn::blob_from_image(
        &padded_mat,
        1.0 / 255.0,
        opencv::core::Size_ {
            width: 640,
            height: 640,
        },
        core::Scalar::new(0f64, 0f64, 0f64, 0f64),
        true,
        false,
        core::CV_32F,
    )?;
    let out_layer_names = model.get_unconnected_out_layers_names()?;
    let mut outs: opencv::core::Vector<core::Mat> = opencv::core::Vector::default();
    model.set_input(&blob, "", 1.0, core::Scalar::default())?;
    model.forward(&mut outs, &out_layer_names)?;
    let detections = post_process(&outs, &mat_info, conf_thresh, nms_thresh)?;
    Ok(detections)
}
/// Prepare an image as a squared matrix.
fn prepare_input(img: &core::Mat) -> opencv::Result<core::Mat> {
    let width = img.cols();
    let height = img.rows();
    let _max = std::cmp::max(width, height);
    let mut result = opencv::core::Mat::zeros(_max, _max, opencv::core::CV_8UC3)
        .unwrap()
        .to_mat()
        .unwrap();
    img.copy_to(&mut result)?;
    Ok(result)
}
/// Process predictions and apply NMS.
fn post_process(
    outs: &core::Vector<core::Mat>,
    mat_info: &MatInfo,
    conf_thresh: f32,
    nms_thresh: f32,
) -> opencv::Result<Detections> {
    let mut det = outs.get(0)?;
    let rows = *det.mat_size().get(1).unwrap();
    let cols = *det.mat_size().get(2).unwrap();
    let mut boxes: core::Vector<opencv::core::Rect> = core::Vector::default();
    let mut scores: core::Vector<f32> = core::Vector::default();
    let mut indices: core::Vector<i32> = core::Vector::default();
    let mut class_index_list: core::Vector<i32> = core::Vector::default();
    let x_factor = mat_info.width / mat_info.scaled_size;
    let y_factor = mat_info.height / mat_info.scaled_size;
    let mut min_val = Some(0f64);
    let mut max_val = Some(0f64);
    let mut min_loc = Some(core::Point::default());
    let mut max_loc = Some(core::Point::default());
    let mut mask = core::no_array();
    let data = det.ptr_mut(0)?.cast::<c_void>();
    let m = unsafe {
        // safe alternative needed..
        core::Mat::new_rows_cols_with_data(rows, cols, core::CV_32F, data, core::Mat_AUTO_STEP)?
    };
    for r in 0..m.rows() {
        let cx: &f32 = m.at_2d::<f32>(r, 0)?;
        let cy: &f32 = m.at_2d::<f32>(r, 1)?;
        let w: &f32 = m.at_2d::<f32>(r, 2)?;
        let h: &f32 = m.at_2d::<f32>(r, 3)?;
        let sc: &f32 = m.at_2d::<f32>(r, 4)?;
        let score = *sc as f64;
        if score < conf_thresh.into() {
            continue;
        }
        let confs = m
            .row(r)?
            .col_range(&core::Range::new(5, m.row(r)?.cols())?)?;
        let c = (confs * score).into_result()?.to_mat()?;
        // find predicted class with highest confidence
        core::min_max_loc(
            &c,
            min_val.as_mut(),
            max_val.as_mut(),
            min_loc.as_mut(),
            max_loc.as_mut(),
            &mut mask,
        )?;
        scores.push(max_val.unwrap() as f32);
        class_index_list.push(max_loc.unwrap().x);
        boxes.push(core::Rect {
            x: (((*cx) - (*w) / 2.0) * x_factor).round() as i32,
            y: (((*cy) - (*h) / 2.0) * y_factor).round() as i32,
            width: (*w * x_factor).round() as i32,
            height: (*h * y_factor).round() as i32,
        });
        indices.push(r);
    }
    // Run NMS.
    dnn::nms_boxes(
        &boxes,
        &scores,
        conf_thresh,
        nms_thresh,
        &mut indices,
        1.0,
        0,
    )?;
    let mut final_boxes: Vec<BoxDetection> = Vec::default();
    for i in &indices {
        let indx = i as usize;
        let class = class_index_list.get(indx)?;
        let rect = boxes.get(indx)?;
        let bbox = BoxDetection {
            xmin: rect.x,
            ymin: rect.y,
            xmax: rect.x + rect.width,
            ymax: rect.y + rect.height,
            conf: scores.get(indx)?,
            class,
        };
        final_boxes.push(bbox);
    }
    Ok(Detections {
        detections: final_boxes,
    })
}
/// Draw predicted bounding boxes.
pub fn draw_predictions(img: &mut core::Mat, detections: &Detections) -> opencv::Result<Vec<u8>> {
    let boxes = &detections.detections;
    let bg_color = core::Scalar::all(255.0);
    let text_color = core::Scalar::all(0.0);
    for i in 0..boxes.len() {
        let bbox = &boxes[i];
        let rect = opencv::core::Rect::new(
            bbox.xmin,
            bbox.ymin,
            bbox.xmax - bbox.xmin,
            bbox.ymax - bbox.ymin,
        );
        let class_names = vec!["object"];
        let label = class_names.get(bbox.class as usize).unwrap();
        opencv::imgproc::rectangle(img, rect, bg_color, 1, opencv::imgproc::LINE_8, 0)?;
        // draw text box above bbox
        let text_size = opencv::imgproc::get_text_size(
            label,
            opencv::imgproc::FONT_HERSHEY_SIMPLEX,
            0.6,
            1,
            &mut 0,
        )
        .unwrap();
        opencv::imgproc::rectangle(
            img,
            core::Rect {
                x: rect.x,
                y: std::cmp::max(0, rect.y - text_size.height - 2),
                width: rect.width,
                height: text_size.height + 2,
            },
            bg_color,
            -1,
            opencv::imgproc::LINE_8,
            0,
        )?;
        opencv::imgproc::put_text(
            img,
            label,
            core::Point {
                x: rect.x,
                y: std::cmp::max(0, rect.y - 2),
            },
            opencv::imgproc::FONT_HERSHEY_SIMPLEX,
            0.6,
            text_color,
            1,
            opencv::imgproc::LINE_AA,
            false,
        )
        .unwrap()
    }
    let mut out_vector: core::Vector<u8> = core::Vector::default();
    opencv::imgcodecs::imencode(".jpg", img, &mut out_vector, &core::Vector::default()).unwrap();
    Ok(out_vector.to_vec())
}
pub fn read_img_to_mat_from_memory(buf: &[u8])-> Result<core::Mat,anyhow::Error>{
    let v8 = VectorOfu8::from_slice(buf);
    let mat = opencv::imgcodecs::imdecode(&v8, opencv::imgcodecs::IMREAD_COLOR)?;
    Ok(mat)
}

pub fn read_img_to_mat_from_path(filename:&str)-> Result<core::Mat,anyhow::Error> {
    let mat = opencv::imgcodecs::imread(filename, opencv::imgcodecs::IMREAD_COLOR)?;
    Ok(mat)
}
// 剪切
pub fn crop(m: &core::Mat,xmin: i32,ymin:i32,xmax: i32,ymax:i32) ->Result<core::Mat,anyhow::Error>{
    let width = m.cols();
    let height = m.rows();
    let ymin = ymin.max(0);
    let xmin = xmin.max(0);
    let ymax = ymax.min(height);
    let xmax = xmax.min(width);
    let row_range = core::Range::new(ymin, ymax)?;
    let col_range = core::Range::new(xmin, xmax)?;
    match core::Mat::rowscols(m, &row_range, &col_range) {
        Ok(t) => Ok(t),
        Err(e) => anyhow::bail!(e.message),
    }
}
pub fn encode(img: &core::Mat)->Result<Vec<u8>,anyhow::Error>{
    let v32 = VectorOfi32::default();
    let mut buf= VectorOfu8::default();
    let res =opencv::imgcodecs::imencode(".jpg", img, &mut buf, &v32);
    
    match res {
        Ok(_) =>  Ok(buf.to_vec()),
        Err(_) => bail!(r#"转换图片失败"#),
    }
}
/// Porting of letterbox padding strategy used to prepare the input image.
///
/// See: https://github.com/ultralytics/yolov5/blob/master/utils/augmentations.py#L91
// fn letterbox(img: &core::Mat, new_shape: core::Size, scale_up: bool) -> opencv::Result<core::Mat> {
//     let width = img.cols() as f32;
//     let height = img.rows() as f32;
//     let new_width = new_shape.width as f32;
//     let new_height = new_shape.height as f32;
//     let mut r = f32::min(new_width / width, new_height / height);
//     if !scale_up {
//         r = f32::min(r, 1.0);
//     }
//     let new_unpad_w = (width * r).round() as i32;
//     let new_unpad_h = (height * r).round() as i32;
//     let dw = (new_shape.width - new_unpad_w) / 2;
//     let dh = (new_shape.height - new_unpad_h) / 2;
//     let mut dst = core::Mat::default();
//     opencv::imgproc::resize(
//         &img,
//         &mut dst,
//         core::Size_ {
//             width: new_unpad_w,
//             height: new_unpad_h,
//         },
//         0.0,
//         0.0,
//         opencv::imgproc::INTER_LINEAR,
//     )?;
//     let top = (dh as f32 - 0.1).round() as i32;
//     let bottom = (dh as f32 + 0.1).round() as i32;
//     let left = (dw as f32 - 0.1).round() as i32;
//     let right = (dw as f32 + 0.1).round() as i32;
//     let mut final_mat = core::Mat::default();
//     opencv::core::copy_make_border(
//         &dst,
//         &mut final_mat,
//         top,
//         bottom,
//         left,
//         right,
//         opencv::core::BORDER_CONSTANT,
//         opencv::core::Scalar::new(114.0, 114.0, 114.0, 114.0),
//     )?;
//     //let params: core::Vector<i32> = core::Vector::default();
//     //opencv::imgcodecs::imwrite("padded.jpg", &final_mat, &params)?;
//     Ok(final_mat)
// }
#[cfg(test)]
mod test {
    use anyhow::Ok;
    use opencv::dnn::{NetTrait, DNN_BACKEND_OPENCV, DNN_TARGET_CPU};
    use std::io::Write;
    use super::{detect, draw_predictions, Yolov5, crop};
    #[test]
    fn test_yolo() -> Result<(), anyhow::Error> {
        let mut net = opencv::dnn::read_net_from_onnx("/home/ibuddy/zfuqiong/yolov5/v6/best.onnx")?;
        net.set_preferable_backend(DNN_BACKEND_OPENCV as i32)?;
        net.set_preferable_target(DNN_TARGET_CPU as i32)?;
        let img = "/home/ibuddy/zfuqiong/tris/testdata/20180827-15-28-11-721.jpg";
        let mut mat = opencv::imgcodecs::imread(img, opencv::imgcodecs::IMREAD_COLOR)?;
        let conf_thresh = 0.5;
        let nms_thresh = 0.5;
        let detections = detect(&mut net, &mat, conf_thresh, nms_thresh)?;
        let img_with_boxes_bytes = draw_predictions(&mut mat, &detections).unwrap();
        let mut file = std::fs::File::create("data.jpg").expect("create failed");
        file.write_all(img_with_boxes_bytes.as_slice()).unwrap();
        dbg!(detections);
        Ok(())
    }
    #[test]
    fn test_detection() -> Result<(), anyhow::Error>{
        let onnx_file = "/home/ibuddy/zfuqiong/yolov5/v6/best.onnx";
        let mut yolo = Yolov5::new(onnx_file.into())?;
        let conf_thresh = 0.5;
        let nms_thresh = 0.5;
        let img = "/home/ibuddy/zfuqiong/tris/testdata/20180827-15-28-11-721.jpg";
        let mut mat = opencv::imgcodecs::imread(img, opencv::imgcodecs::IMREAD_COLOR)?;
        let detects = yolo.detect(&mat, conf_thresh, nms_thresh)?;
        dbg!(detects);
        Ok(())
    }
    #[test]
    fn test_crop() -> Result<(), anyhow::Error>{
        let onnx_file = "/home/ibuddy/zfuqiong/yolov5/v6/best.onnx";
        let mut yolo = Yolov5::new(onnx_file.into())?;
        let conf_thresh = 0.5;
        let nms_thresh = 0.5;
        let img = "/home/ibuddy/zfuqiong/tris/testdata/20180827-15-28-11-721.jpg";
        let mut mat = opencv::imgcodecs::imread(img, opencv::imgcodecs::IMREAD_COLOR)?;
        let detects = yolo.detect(&mat, conf_thresh, nms_thresh)?;
        for (i, det) in detects.detections.into_iter().enumerate() {
            let smat = crop(&mat, det.xmin, det.ymin, det.xmax, det.ymax)?;
            let params = opencv::core::Vector::<i32>::default();
            opencv::imgcodecs::imwrite(format!("{}.jpg",i).as_str(), &smat, &params)?;
        }
        Ok(())
    }
}
