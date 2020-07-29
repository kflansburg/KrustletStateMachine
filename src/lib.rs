#[macro_use]
pub mod state;
// pub mod concrete_provider;
// pub mod concrete_state_machine;
pub mod kubelet;
pub mod state_machine;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {}
}
