use crate::HappyNewYearArgs;

pub fn happy_new_year(args: HappyNewYearArgs) -> eyre::Result<()> {
    println!("🎉 {}", args.message);
    Ok(())
}
