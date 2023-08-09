use rethers::*;

#[tokio::main]
async fn main() {

  let test_algo = TestAlgo::new();

  let mut fw: Framework<TestAlgo> = Framework::new::<TestAlgo>(test_algo);


  fw.run().await;
}