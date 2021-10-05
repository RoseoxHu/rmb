# -*- coding: utf-8 -*-

import os, sys
import logging

'''
人民币大写转换
'''

'''数字映射''' 
num_map = {
    0: '零',
    1: '壹',
    2: '贰',
    3: '叁',
    4: '肆',
    5: '伍',
    6: '陆',
    7: '柒',
    8: '捌',
    9: '玖',
}

'''单位映射'''
unit_map = {
    0: '分',
    1: '角',
    2: '元',
    3: '拾',
    4: '佰',
    5: '仟',
    6: '萬',
    7: '拾',
    8: '佰',
    9: '仟',
    10: '亿',
    11: '拾',
    12: '佰',
    13: '仟',
}

def to_rmb(amount):
    ''' 转人民币大写 '''
    amount = round(amount * 100) # 金额转为分
    
    result = ''
    pointer = 0        # 单位指针
    last_remainder = 0 # 上一个余数
    while (amount > 0):
        remainder = int(amount % 10) # 余数
        amount = int(amount / 10)    # 缩小10倍
        logging.debug("amount: %s, remainder: %s, pointer: %s" % (amount, remainder, pointer))

        if remainder == 0:
            if (pointer == 2 or pointer == 6 or pointer == 10):
                # 余数为0, 元、萬、亿单位需要保留
                result = '%s%s' % (unit_map[pointer], result)
            else:
                if last_remainder > 0:
                    # 余数为0, 上一个余数也为0, 避免零仟零佰
                    result = '%s%s' % (num_map[remainder], result)
        else:
            result = '%s%s%s' % (num_map[remainder], unit_map[pointer], result)
        
        last_remainder = remainder
        pointer += 1    # 从右往左
        if pointer == 2 and result == '':  # 无角、分尾数
            result = '整'
        print(result)
    return result


if __name__ == "__main__":
    if len(sys.argv) < 1:
        print('Usage: %s <金额>' % os.path.abspath(sys.argv[0]))
        sys.exit(0)
    
    logging.basicConfig(level=logging.NOTSET, 
        format='%(asctime)s - %(filename)s[line:%(lineno)d/%(thread)d] - %(levelname)s: %(message)s') # 设置日志级别
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    logging.debug("os.getcwd() = %s" % os.getcwd())

    try:
        amount = round(float(sys.argv[1]) * 100) / 100.0
        if amount >= 1000000000000:
            print('Usage: %s 金额超出范围[0, 1000000000000)' % os.path.abspath(sys.argv[0]))
            sys.exit(0)
    except Exception:
        print('Usage: %s <金额>' % os.path.abspath(sys.argv[0]))
        sys.exit(0)
    logging.debug("amount: %s" % amount)
    logging.debug("num_map: %s" % num_map)
    logging.debug("unit_map: %s" % unit_map)

    to_rmb(amount)
