use crate::*;

pub fn handle_error(tmp: &Tmp, err_string: &str) {
    tmp.get_log().error(err_string, common_log);
}
