#!/bin/sh
###########################################################
######### 人民币大写转换
######### bash 不支持浮点运算，需要借助bc,awk处理浮点运算
###########################################################

if [[ $# < 1 ]]; then
    echo "Usage $0 <amount>"
    exit
fi
amount=$1

# bc对于加减乘三种算法依据输入中的最高精度来确定输出精度，
# 不会进行自行截断，此时scale设置无效；但是除法则相反。
# echo "scale=2; 123.45678 * 100 / 1.0" | bc
# 使用printf来控制输出精度，默认做了四舍五入
#let amount*=100 # 转为分
amount=$(printf "%.0f" $(echo "$amount * 100" | bc))
if [ $amount -ge 100000000000000 ]; then
    echo "Usage: $1 金额超出范围[0, 10000000000)"
    exit
fi
echo -n "金额: "
echo "scale=2;$amount / 100" | bc

# 大写数字
declare -a num_arr=(零 壹 贰 叁 肆 伍 陆 柒 捌 玖)
# 数字单位
declare -a unit_arr=(分 角 元 拾 佰 仟 萬 拾 佰 仟 亿 拾 佰 仟)

#echo ${num_arr[*]}
#for num in ${num_arr[*]}
#do
#  echo $num
#done

#echo ${unit_arr[*]}
#for unit in ${unit_arr[*]}
#do
#  echo $unit
#done

result=''
pointer=0        # 单位指针
last_remainder=0 # 上一个余数
pid=$$           # 脚本运行的当前进程ID号
while (( $amount > 0 ))
do
    remainder=$(( $amount % 10 )) # 余数
    amount=$[ $amount / 10 ]    # 缩小10倍

    now=$(date '+%F %T')
    printf "[%s]-[%d] amount: %s, remainder: %s, pointer: %s\n" \
        "$now" $pid $amount $remainder $pointer

    if [ $remainder -eq 0 ]; then
       if [ $pointer -eq 2 -o $pointer -eq 6 -o $pointer -eq 10 ]; then
           # 余数为0, 元、萬、亿单位需要保留
           result="${unit_arr[$pointer]}$result"
       else
           if [ $last_remainder -gt 0 ]; then
               # 余数为0, 上一个余数也为0, 避免零仟零佰
               result="${num_arr[$remainder]}$result"
           fi
       fi
    else
        result="${num_arr[$remainder]}${unit_arr[$pointer]}$result"
    fi

    last_remainder=$remainder
    let pointer+=1    # 从右往左
    if [ $pointer -eq 2 -a "$result" == "" ]; then # 无角、分尾数
        result="整"
    fi
    printf "[%s]-[%d] $result\n" "$now" $pid
done # end while
