use futures::join;

pub mod day1;

pub async fn year_run() {
    join![day1::run()];
}
