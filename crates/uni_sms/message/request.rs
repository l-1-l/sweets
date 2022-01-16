use serde::Serialize;

use super::{MessageKind, UniMobile};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniRequest<T> {
    to: UniMobile,
    signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_data: Option<T>,
}
impl<T: Serialize> UniRequest<T> {
    pub fn new(
        to: UniMobile,
        signature: String,
        kind: MessageKind,
        template_data: Option<T>,
    ) -> Self {
        let mut content: Option<String> = None;
        let mut template_id: Option<String> = None;

        match kind {
            MessageKind::Custom(_) => {
                content = Some(kind.to_string());
            }
            MessageKind::Template(_) => {
                template_id = Some(kind.to_string());
            }
        }

        Self {
            to,
            signature,
            content,
            template_id,
            template_data,
        }
    }
}
