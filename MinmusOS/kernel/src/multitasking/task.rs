// Project Name:  MinmusOS
// File Name:     task.rs
// File Function: Task manager
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;

pub const MAX_TASKS: i8 = 32;
const STACK_SIZE: usize = 4096;

#[repr(C, packed)]
pub struct CPUState {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Task {
    pub stack: [u8; STACK_SIZE],
    pub cpu_state_ptr: u32,
    pub running: bool,
}

static NULL_TASK: Task = Task {
    stack: [0; STACK_SIZE],
    cpu_state_ptr: 0u32,
    running: false,
};

impl Task {
    pub fn init(&mut self, entry_point: u32) {
        self.running = true;
        let mut state = &self.stack as *const u8;
        unsafe {
            state = state.byte_add(STACK_SIZE);
            state = state.byte_sub(size_of::<CPUState>());
        }
        self.cpu_state_ptr = state as u32;
        let cpu_state = self.cpu_state_ptr as *mut CPUState;
        unsafe {
            (*cpu_state).eax = 0;
            (*cpu_state).ebx = 0;
            (*cpu_state).ecx = 0;
            (*cpu_state).edx = 0;
            (*cpu_state).esi = 0;
            (*cpu_state).edi = 0;
            (*cpu_state).ebp = 0;
            (*cpu_state).eip = entry_point;
            (*cpu_state).cs = 0x8;
            (*cpu_state).eflags = 0x202;
        }
    }
}

pub struct TaskManager {
    pub(crate) tasks: [Task; MAX_TASKS as usize],
    task_count: i8,
    current_task: i8,
}

pub static mut TASK_MANAGER: TaskManager = TaskManager {
    tasks: [NULL_TASK; MAX_TASKS as usize],
    task_count: 0,
    current_task: -1,
};

impl TaskManager {
    pub fn init(&mut self) {
        self.add_task(idle as u32);
    }

    pub fn add_task(&mut self, entry_point: u32) {
        self.tasks[self.get_free_slot() as usize].init(entry_point);
        self.task_count += 1;
    }

    pub fn remove_task(&mut self, id: usize) {
        if id != 0 {
            self.tasks[id] = NULL_TASK;
            self.task_count -= 1;
        }
    }

    pub fn remove_current_task(&mut self) {
        self.remove_task(self.current_task as usize);
    }

    pub fn schedule(&mut self, cpu_state: *mut CPUState) -> *mut CPUState {
        if self.task_count <= 0 {
            return cpu_state;
        }
        if self.current_task >= 0 {
            self.tasks[self.current_task as usize].cpu_state_ptr = cpu_state as u32;
        }
        self.current_task = self.get_next_task();
        self.tasks[self.current_task as usize].cpu_state_ptr as *mut CPUState
    }

    pub fn get_next_task(&self) -> i8 {
        let mut i = self.current_task + 1;
        while i < MAX_TASKS {
            if self.tasks[i as usize].running {
                return i;
            }
            i = (i + 1) % MAX_TASKS;
        }
        -1
    }

    pub fn get_free_slot(&self) -> i8 {
        let mut slot: i8 = -1;
        for i in 0..MAX_TASKS {
            if self.tasks[i as usize].running == false {
                slot = i;
                return slot;
            }
        }
        slot
    }

    pub fn get_current_slot(&self) -> i8 {
        self.current_task
    }

    pub fn list_tasks(&self) {
        lib::println!("Running tasks:");
        for i in 0..MAX_TASKS {
            if self.tasks[i as usize].running {
                lib::println!("PID: {}", i);
            }
        }
    }
}

fn idle() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}