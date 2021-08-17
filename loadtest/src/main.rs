use goose::prelude::*;

async fn loadtest_bar(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("hello_world").await?;

    Ok(())
}

fn main() -> Result<(), GooseError> {
    let _goose_metrics = GooseAttack::initialize()?
        .register_taskset(taskset!("LoadtestTasks")
            // Register the bar task, assigning it a weight of 2 (so it
            // runs 1/5 as often as bar). Apply a task name which shows up
            // in metrics.
            .register_task(task!(loadtest_bar).set_name("Hola_Mundo").set_weight(2)?)
        )
        // You could also set a default host here, for example:
        .set_default(GooseDefault::Host, "http://localhost:8000/api/")?
        // We set a default run time so this test runs to completion.
        .set_default(GooseDefault::RunTime, 40)?
        .set_default(GooseDefault::Users, 35)?
        .execute()?;
        // .print();

    Ok(())
}
