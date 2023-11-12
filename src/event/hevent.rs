use crate::event::hloop::Hloop;
use crate::event::hloop::HloopStatus;

#[derive(Copy)]
pub enum HeventType {
    None,   // 无类型
    Io,     // IO事件
    Time,   // 时间事件
    Period, // 周期事件
    Timer,  // 定时器事件
    Idle,   // 空闲事件
    Custom, // 自定义事件
}

impl Clone for HeventType {
    fn clone(&self) -> Self {
        match self {
            HeventType::None => HeventType::None,
            HeventType::Io => HeventType::Io,
            HeventType::Time => HeventType::Time,
            HeventType::Period => HeventType::Period,
            HeventType::Timer => HeventType::Timer,
            HeventType::Idle => HeventType::Idle,
            HeventType::Custom => HeventType::Custom,
        }
    }
}

pub struct Hevent<'a> {
    hloop: &'a Hloop<'a>,                                 // 指向事件循环管理器
    pub id: u64,                                          // 事件ID
    pub event_callback: Option<&'a fn()>,                 // 事件回调函数
    pub event_callback_userdata: Option<&'a dyn FnMut()>, // 事件回调函数的私有数据
    pub event_callback_privdata: Option<&'a dyn FnMut()>, // 事件回调函数的私有数据
    pub status: HloopStatus,                              // 事件状态
    pub priority: u32,                                    // 事件优先级
    pub htype: HeventType,                                // 事件类型
    pub pending_next: Option<&'a Hevent<'a>>,             // 指向下一个挂起的事件
}

impl<'a> Hevent<'a> {
    pub fn new(hloop: &'a Hloop<'a>) -> Hevent<'a> {
        Hevent {
            hloop,
            id: 0,
            event_callback: None,
            event_callback_userdata: None,
            event_callback_privdata: None,
            status: HloopStatus::Stop,
            priority: 0,
            htype: HeventType::None,
            pending_next: None,
        }
    }
}

impl<'a> PartialEq for Hevent<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<'a> PartialOrd for Hevent<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<'a> Eq for Hevent<'a> {}

impl<'a> Ord for Hevent<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<'a> Clone for Hevent<'a> {
    fn clone(&self) -> Self {
        let mut hevent = Hevent::new(self.hloop);
        hevent.id = self.id;
        hevent.event_callback = self.event_callback;
        hevent.event_callback_userdata = self.event_callback_userdata;
        hevent.event_callback_privdata = self.event_callback_privdata;
        hevent.status = self.status;
        hevent.priority = self.priority;
        hevent.htype = self.htype;
        hevent.pending_next = self.pending_next;
        hevent
    }
}
