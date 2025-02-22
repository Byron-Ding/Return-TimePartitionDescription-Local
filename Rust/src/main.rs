use chrono;

struct TimePartition{
    partition_chinese_name: &'static str,
    start: Option<chrono::NaiveTime>,
    end: Option<chrono::NaiveTime>,
}

impl TimePartition {
    fn cross_day(&self) -> Option<bool> {
        if self.start.is_none() || self.end.is_none() {
            return None
        }
        
        return Some(self.start.unwrap() > self.end.unwrap());
    }

    fn is_in_partition(&self, time: chrono::NaiveTime) -> Option<bool> {
        if self.start.is_none() || self.end.is_none() {
            return None;
        }

        if self.cross_day().unwrap() {
            if time >= self.start.unwrap() || time <= self.end.unwrap() {
                return Some(true)
            }
        } else {
            if time >= self.start.unwrap() && time <= self.end.unwrap() {
                return Some(true);
            }
        }

        return Some(false);
    }

    fn get_partition_chinese_name(&self) -> &'static str {
        return self.partition_chinese_name;
    }
}

/* 
time_partition = {
    '凌晨': [datetime.time(1, 0), datetime.time(5, 0)],
    '清晨': [datetime.time(5, 0), datetime.time(7, 0)],
    '早晨': [datetime.time(7, 0), datetime.time(9, 0)],
    '上午': [datetime.time(9, 0), datetime.time(11, 0)],
    '中午': [datetime.time(11, 0), datetime.time(13, 0)],
    '下午': [datetime.time(13, 0), datetime.time(16, 30)],
    '傍晚': [datetime.time(16, 30), datetime.time(17, 0)],
    '黄昏': [datetime.time(17, 0), datetime.time(18, 30)],
    '晚上': [datetime.time(18, 30), datetime.time(22, 0)],
    '深夜': [datetime.time(22, 0), datetime.time(23, 30)],
}
*/

const TIME_PARTITION_A_DAY: [TimePartition; 11] = [
    TimePartition{
        partition_chinese_name: "凌晨",
        start: chrono::NaiveTime::from_hms_opt(1, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(5, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "清晨",
        start: chrono::NaiveTime::from_hms_opt(5, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(7, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "早晨",
        start: chrono::NaiveTime::from_hms_opt(7, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(9, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "上午",
        start: chrono::NaiveTime::from_hms_opt(9, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(11, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "中午",
        start: chrono::NaiveTime::from_hms_opt(11, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(13, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "下午",
        start: chrono::NaiveTime::from_hms_opt(13, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(16, 30, 0),
    },
    TimePartition{
        partition_chinese_name: "傍晚",
        start: chrono::NaiveTime::from_hms_opt(16, 30, 0),
        end: chrono::NaiveTime::from_hms_opt(17, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "黄昏",
        start: chrono::NaiveTime::from_hms_opt(17, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(18, 30, 0),
    },
    TimePartition{
        partition_chinese_name: "晚上",
        start: chrono::NaiveTime::from_hms_opt(18, 30, 0),
        end: chrono::NaiveTime::from_hms_opt(22, 0, 0),
    },
    TimePartition{
        partition_chinese_name: "深夜",
        start: chrono::NaiveTime::from_hms_opt(22, 0, 0),
        end: chrono::NaiveTime::from_hms_opt(23, 30, 0),
    },
    TimePartition{
        partition_chinese_name: "深夜",
        start: chrono::NaiveTime::from_hms_opt(23, 30, 0),
        end: chrono::NaiveTime::from_hms_opt(1, 0, 0),
    },
];


fn get_time_partition() -> Option<&'static TimePartition> {
    let current_naive_time: chrono::NaiveTime = chrono::Local::now().time();

    for time_partition in TIME_PARTITION_A_DAY.iter() {
        if time_partition.is_in_partition(current_naive_time)? {
            return Some(time_partition);
        }
    }
    return  None;
    
}

fn main() {
    // judge the time partition
    let time_partition: Option<&TimePartition> = get_time_partition();
    
    match time_partition {
        Some(time_partition) => {
            println!("{}", time_partition.get_partition_chinese_name());
        },
        None => {
            println!("[错误]");
        }
    }
}


