pub struct Client<'a> {
    pub(crate) client: reqwest::Client,
    pub(crate) token: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            token,
        }
    }

    pub fn api<T: FromClient<'a>>(&'a self) -> T {
        T::from_client(self)
    }
}

pub trait FromClient<'a> {
    fn from_client(client: &'a Client) -> Self;
}
