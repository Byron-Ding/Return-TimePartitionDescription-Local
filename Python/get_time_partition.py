import datetime



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

# 判断当前时间属于哪个时间段
def get_time_partition():
    now = datetime.datetime.now().time()
    # 处理跨天的情况
    if now < datetime.time(1, 0) or now >= datetime.time(23, 30):
        return '午夜'

    # 遍历时间段
    for time_name, time_range in time_partition.items():
        if time_range[0] <= now < time_range[1]:
            return time_name
        
    raise ValueError("Time out of range")

if __name__ == "__main__":
    print(get_time_partition())
    