use kalosm_llama::prelude::*;

#[tokio::main]
async fn main() {
    let mut model = Llama::builder()
        .with_source(LlamaSource::mistral_7b())
        .build()
        .unwrap();
    model
        .run_sync(move |model| {
            Box::pin(async move {
                let prompt = "The capital of France is ";
                let mut session = model.new_session().unwrap();
                let logits = model.feed_text(&mut session, prompt).unwrap();
                println!("{:?}", logits);
                let prompt = "paris";
                println!("{:?}", model.tokenizer().encode(prompt));
                let logits = model.feed_text(&mut session, prompt).unwrap();
                println!("{:?}", logits);
            })
        })
        .await
        .unwrap();
}