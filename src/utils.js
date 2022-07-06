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
  


  export {
    get_file_name,
    ext
  }