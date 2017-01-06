use std::borrow::Cow;

/// SHIORIレスポンスの作成
/// # ステータスコード
/// ## 2xx - 処理完了
/// * 200 OK
///   正常に終了した
/// * 204 No Content
///   正常に終了したが、返すべきデータがない
///
/// ## 3xx - 処理完了、追加アクション要求
/// * 310 Communicate
///   - deprecated -
/// * 311 Not Enough
///   TEACH リクエストを受けたが、情報が足りない
/// * 312 Advice
///   TEACH リクエスト内の最も新しいヘッダが解釈不能
///
/// ## 4xx - リクエストエラー
/// * 400 Bad Request
///   リクエスト不備
///
/// ## 5xx - サーバエラー
/// * 500 Internal Server Error
///   サーバ内でエラーが発生した
pub struct ShioriResponse {}

fn make_response(header: &str, value: &str, refs: &[&str], kv: &[(&str, &str)]) -> String {
    let mut buf = header.to_string();
    if !value.is_empty() {
        buf.push_str("Value: ");
        buf.push_str(value);
        buf.push_str("\r\n");
    }
    for item in kv {
        let (k, v) = *item;
        buf.push_str(k);
        buf.push_str(": ");
        buf.push_str(v);
        buf.push_str("\r\n");
    }
    for (i, v) in refs.into_iter().enumerate() {
        buf.push_str(&format!("Reference{}: ", i));
        buf.push_str(v);
        buf.push_str("\r\n");
    }
    buf.push_str("\r\n");
    buf
}

static NO_CONTENT_STRING: &'static str = concat!(
    "SHIORI/3.0 204 No Content\r\n",
    "Charset: UTF-8\r\n",
    "\r\n",
);
static OK_STRING: &'static str = concat!(
    "SHIORI/3.0 200 OK\r\n",
    "Charset: UTF-8\r\n",
);
static BAD_REQUEST_STRING: &'static str = concat!(
    "SHIORI/3.0 400 Bad Request\r\n",
    "Charset: UTF-8\r\n",
);
static INTERNAL_SERVER_ERROR_STRING: &'static str = concat!(
    "SHIORI/3.0 500 Internal Server Error\r\n",
    "Charset: UTF-8\r\n",
);

impl ShioriResponse {
    /// 204 No Content
    pub fn no_content<'a>() -> Cow<'a, str> {
        Cow::Borrowed(NO_CONTENT_STRING)
    }

    /// 200 OK
    pub fn ok<'a>(value: &str, refs: &[&str], kv: &[(&str, &str)]) -> Cow<'a, str> {
        let res = make_response(OK_STRING, value, refs, kv);
        Cow::Owned(res)
    }

    /// 200 OK, talk出力
    pub fn talk<'a>(value: &str) -> Cow<'a, str> {
        let kv = [];
        let refs = [];
        ShioriResponse::ok(value, &refs[..], &kv[..])
    }

    /// 400 Bad Request
    pub fn bad_request<'a>(reason: &str) -> Cow<'a, str> {
        let kv = [("X-PASTA-ERROR-REASON", reason)];
        let refs = [];
        let res = make_response(BAD_REQUEST_STRING, "", &refs[..], &kv[..]);
        Cow::Owned(res)
    }

    /// 500 Internal Server Error
    pub fn internal_server_error<'a>(reason: &str) -> Cow<'a, str> {
        let kv = [("X-PASTA-ERROR-REASON", reason)];
        let refs = [];
        let res = make_response(INTERNAL_SERVER_ERROR_STRING, "", &refs[..], &kv[..]);
        Cow::Owned(res)
    }
}

#[test]
fn res_test() {
    {
        let check = concat!(
            "SHIORI/3.0 204 No Content\r\n",
            "Charset: UTF-8\r\n",
            "\r\n",
        );
        let act = ShioriResponse::no_content();
        assert_eq!(check, act);
    }
    {
        let check = concat!(
            "SHIORI/3.0 200 OK\r\n",
            "Charset: UTF-8\r\n",
            "Value: 惨事の貴方\r\n",
            "Craftman: don't wolly\r\n",
            "Version: 1.2.3.4\r\n",
            "Reference0: 参照０\r\n",
            "Reference1: 参照１\r\n",
            "Reference2: 参照２\r\n",
            "\r\n",
        );
        let value = "惨事の貴方";
        let refs = ["参照０", "参照１", "参照２"];
        let kv = [("Craftman", "don't wolly"), ("Version", "1.2.3.4")];
        let act = ShioriResponse::ok(value, &refs[..], &kv[..]);
        assert_eq!(check, act);
    }
    {
        let check = concat!(
            "SHIORI/3.0 500 Internal Server Error\r\n",
            "Charset: UTF-8\r\n",
            "X-PASTA-ERROR-REASON: sorry\r\n",
            "\r\n",
        );
        let reason = "sorry";
        let act = ShioriResponse::internal_server_error(reason);
        assert_eq!(check, act);
    }
}
