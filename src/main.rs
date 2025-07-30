use std::{process::Command as PsCommand, time::Duration};
use tokio::time::sleep;
use clap::Parser; 

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// time you want to work: minutes
    #[arg(short, long, default_value_t = 20)]
    pub work: u16, 
    /// time you want to rest: minutes
    #[arg(short, long, default_value_t = 5)]
    pub rest: u16, 
    /// cycles
    #[arg(short, long, default_value_t = 1)]
    pub cycle: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut pomodoro = PomodoroMeta::new(args.work * 60, args.rest * 60, args.cycle);


    let work_job = tokio::spawn(async move {
        pomodoro.pomodoro_loop().await;
    });

    work_job.await.unwrap();
}

#[derive(Debug, Clone)]
pub struct PomodoroMeta {
    work: u16, // secs
    rest: u16, // secs
    cycle: u16 // number of times to repeat this process
}

impl PomodoroMeta {
    fn new(work: u16, rest: u16, cycle: u16) -> Self {
        PomodoroMeta { work, rest, cycle }
    }

    async fn pomodoro_loop(&mut self) {
        while self.cycle != 0 {
            while self.work != 0 {
                sleep(Duration::from_secs(1)).await;
                self.work -= 1;
                println!("working time left {}: mins {}: secs", self.work / 60, self.work % 60);
            }
            make_beep(1).await;

            while self.rest != 0 {
                sleep(Duration::from_secs(1)).await;
                self.rest -= 1;
                println!("resting time left {}: mins {}: secs", self.rest / 60, self.rest % 60);
            }

            make_beep(2).await;
            self.cycle -= 1
        }
        sleep(Duration::from_secs(5)).await;
        make_beep(3).await;
    }
}

async fn make_beep(times: u8) {
    for _ in 0..times {
        let mut command = PsCommand::new("beep");
    
        let output = command.output().unwrap(); 
        if !output.status.success() {
            panic!("could not beep")
        }
        sleep(Duration::from_secs(1)).await;
    }
    println!("change mode")
}


