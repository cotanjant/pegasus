use pegasus::System;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sys_data = System::get_system_data().await?;
    println!("{:?}", sys_data);
    Ok(())
}