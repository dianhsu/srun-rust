mod srun;
fn main() {
    let login_info = srun::LoginInfo {
        callback: "jsonp1583251661368".to_string(),
        action: "login".to_string(),
        username: "202013703018".to_string(),
        password: "xudian1234..".to_string(),
        ac_id: "7".to_string(),
        enc: "srun_bx1".to_string(),
        info: "".to_string(),
        chksum: "".to_string(),
        n: "".to_string(),
        vtype: "".to_string(),
        interface: "".to_string(), 
        host: "".to_string(),
        https: false
    };
    login_info.login();
}
