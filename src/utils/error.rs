use crate::*;

pub fn handle_error<T: ToString>(tmp: &Tmp, err_string: T) {
    tmp.get_log().error(err_string, common_log);
}
