use super::hevent::Hevent;
use queue::Queue;
use std::collections::BinaryHeap;

/// ## 循环状态
/// 有三个状态：
/// 1. `Running`: 正在运行
/// 2. `Stop`: 停止
/// 3. `Pause`: 暂停
#[derive(Copy)]
pub enum HloopStatus {
    Running,
    Stop,
    Pause,
}

impl Clone for HloopStatus {
    fn clone(&self) -> Self {
        match self {
            HloopStatus::Running => HloopStatus::Running,
            HloopStatus::Stop => HloopStatus::Stop,
            HloopStatus::Pause => HloopStatus::Pause,
        }
    }
}

pub struct Hloop<'a> {
    pub flags: u32,           // 事件循环的标志
    pub status: HloopStatus,  // 事件循环的状态
    pub start_ms: u64,        // 起始时间戳
    pub start_hrtime_us: u64, // 起始高精度时间戳
    pub stop_hrtime_us: u64,  // 停止高精度时间戳
    pub cur_hrtime_us: u64,   // 当前高精度时间戳
    pub loop_cnt: u64,        // 事件循环的次数
    pub pid: u64,             // 进程ID
    pub tid: u64,             // 线程ID

    pendings: Vec<Hevent<'a>>,      // 挂起的事件列表，应当按优先级排序
    idles: Vec<Hevent<'a>>,         // 空闲事件列表，应当按优先级排序
    timers: BinaryHeap<Hevent<'a>>, // 定时器事件列表，应当按超时时间排序
    ios: Vec<Hevent<'a>>,           // IO事件列表，应当按优先级排序
    customes: Queue<Hevent<'a>>,    // 自定义事件列表，应当按优先级排序

    customes_mutex: std::sync::Mutex<bool>, // 自定义事件列表的互斥锁
}

impl<'a> Hloop<'a> {
    pub fn new() -> Hloop<'a> {
        Hloop {
            flags: 0,
            status: HloopStatus::Stop,
            start_ms: 0,
            start_hrtime_us: 0,
            stop_hrtime_us: 0,
            cur_hrtime_us: 0,
            loop_cnt: 0,
            pid: 0,
            tid: 0,
            pendings: Vec::new(),
            idles: Vec::new(),
            timers: BinaryHeap::new(),
            ios: Vec::new(),
            customes: Queue::new(),
            customes_mutex: std::sync::Mutex::new(false),
        }
    }
}
