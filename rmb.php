<?php
/**
 * 人民币大写转换
 */

/**  数字映射*/
$num_map = array(
    0 => '零',
    1 => '壹',
    2 => '贰',
    3 => '叁',
    4 => '肆',
    5 => '伍',
    6 => '陆',
    7 => '柒',
    8 => '捌',
    9 => '玖',
);

/** 单位映射 */
$unit_map = array(
    0 => '分',
    1 => '角',
    2 => '元',
    3 => '拾',
    4 => '佰',
    5 => '仟',
    6 => '萬',
    7 => '拾',
    8 => '佰',
    9 => '仟',
    10 => '亿',
    11 => '拾',
    12 => '佰',
    13 => '仟',
);


/**
 * 转人民币大写
 * @param $amount
 * @return mixed
 */
function to_rmb($amount) {
    global $num_map, $unit_map;
    $amount = round($amount * 100); # 金额转为分
    
    $result = '';
    $pointer = 0;        # 单位指针
    $last_remainder = 0; # 上一个余数
    while ($amount > 0) {
        $remainder = intval($amount % 10); # 余数
        $amount = intval($amount / 10);    # 缩小10倍
        printf("[%s]-[%d] amount: %s, remainder: %s, pointer: %s\n",
            date("Y-m-d H:i:s"), getmypid(), $amount, $remainder, $pointer);

        if ($remainder == 0) {
            if ($pointer == 2 or $pointer == 6 or $pointer == 10) {
                # 余数为0, 元、萬、亿单位需要保留
                $result = $unit_map[$pointer] . $result;
            } else {
                if ($last_remainder > 0) {
                    # 余数为0, 上一个余数也为0, 避免零仟零佰
                    $result = $num_map[$remainder] . $result;
                }
            }
        } else {
            $result = $num_map[$remainder] . $unit_map[$pointer] . $result;
        }
        $last_remainder = $remainder;
        $pointer += 1;    # 从右往左
        if ($pointer == 2 and $result == '') {  # 无角、分尾数
            $result = '整';
        }
        printf("[%s]-[%d] $result\n", date("Y-m-d H:i:s"), getmypid());
    }// end while

    return $result;
}

function main($argv) {
    global $num_map, $unit_map;

    if (count($argv) < 2) {
        printf("Usage: %s <金额>", $argv[0]);
        exit(0);
    }

    try {
        $amount = round(floatval($argv[1]) * 100) / 100.0;
        if ($amount >= 1000000000000) {
            printf('Usage: %s 金额超出范围[0, 1000000000000)', $argv[0]);
            exit(0);
        }
    } catch (Exception $e) {
        printf('Usage: %s <金额>',  $argv[0]);
        exit(0);
    }
    var_dump($amount);
    print_r($num_map);
    print_r($unit_map);

    to_rmb($amount);
}

main($argv);