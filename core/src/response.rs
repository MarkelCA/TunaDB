use crate::proto;

pub enum Status {
    Unespecified,
    NotFound,
    Ok,
    Error,
}

pub struct Response {
    pub status: Status,
    pub content: Option<String>,
}

impl Response {
    pub fn to_proto_response(&self) -> proto::Response {
        let status = match self.status {
            Status::Unespecified => proto::response::Status::Unespecified,
            Status::NotFound => proto::response::Status::NotFound,
            Status::Ok => proto::response::Status::Ok,
            Status::Error => proto::response::Status::Error,
        };
        proto::Response {
            status: status as i32,
            content: self.content.clone(),
        }
    }

    pub fn from_proto_response(proto_response: proto::Response) -> Response {
        match proto_response.status() {
            proto::response::Status::Unespecified => Response {
                status: Status::Unespecified,
                content: Some(proto_response.content().to_string()),
            },
            proto::response::Status::NotFound => Response {
                status: Status::NotFound,
                content: None,
            },
            proto::response::Status::Ok => Response {
                status: Status::Ok,
                content: Some(proto_response.content().to_string()),
            },
            proto::response::Status::Error => Response {
                status: Status::Error,
                content: Some(proto_response.content().to_string()),
            },
        }
    }
}
