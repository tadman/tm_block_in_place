#[tokio::main(worker_threads = 2)]
async fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dont_panic() {
        tokio::task::block_in_place(move || { });
    }
}
