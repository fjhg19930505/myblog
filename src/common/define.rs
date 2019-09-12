#[derive(Debug)]
pub enum VerifyResult {
    VerifyResult_OK,
    VerifyResult_PhoneError,
    VerifyResult_CodeError,
}
