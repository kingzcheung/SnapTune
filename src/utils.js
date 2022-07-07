const get_file_name = (path) => {
  var pos1 = path.lastIndexOf("/");
  var pos2 = path.lastIndexOf("\\");
  var pos = Math.max(pos1, pos2);
  if (pos < 0) return path;
  else return path.substring(pos + 1);
};

const ext = (file_path) => {
  const h = file_path.substr(file_path.lastIndexOf(".") + 1);
  return h.toUpperCase();
};


const get_file_size = (size) => {
  if (!size) return "";
  var num = 1024.00; //byte
  if (size < num)
    return size + "B";
  if (size < Math.pow(num, 2))
    return (size / num).toFixed(2) + "KB"; //kb
  if (size < Math.pow(num, 3))
    return (size / Math.pow(num, 2)).toFixed(2) + "MB"; //M
  if (size < Math.pow(num, 4))
    return (size / Math.pow(num, 3)).toFixed(2) + "GB"; //G
  return (size / Math.pow(num, 4)).toFixed(2) + "TB"; //T

}
export {
  get_file_name,
  ext,
  get_file_size
}