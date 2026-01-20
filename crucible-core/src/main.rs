use crucible_core::IntentAst;

fn main() -> anyhow::Result<()> {
    println!("🔥 Crucible Engine - Correct by Design, Not by Debugging");
    println!("📋 Testing core functionality...");

    let mut ast = IntentAst::new();
    println!("✅ Created new Intent-AST: {}", ast.id);

    ast.add_requirement("User can withdraw money from account".to_string());
    ast.add_requirement("Withdrawal amount must be positive".to_string());
    ast.add_requirement("Account balance must be sufficient".to_string());

    println!("📊 Requirements added: {}", ast.requirements.len());
    println!("🎯 Correctness score: {:.1}%", ast.correctness_score);

    let json = serde_json::to_string_pretty(&ast)?;
    println!("📄 Intent-AST JSON:\n{}", json);

    println!("✅ Core functionality working!");
    Ok(())
}