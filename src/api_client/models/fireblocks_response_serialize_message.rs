pub trait FireblocksSerializebleMessage{
    fn serialize(body: Vec<u8>) -> Self;
}