/**
 * 人民币大写转换
 */
package main

import (
	"bufio"
	"flag"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// 数字映射
var num_arr [10]string

// 单位映射
var unit_map map[int]string

var verbosePtr *bool

func init() {
	num_arr = [10]string{
		"零",
		"壹",
		"贰",
		"叁",
		"肆",
		"伍",
		"陆",
		"柒",
		"捌",
		"玖",
	}

	unit_map = map[int]string{
		0:  "分",
		1:  "角",
		2:  "元",
		3:  "拾",
		4:  "佰",
		5:  "仟",
		6:  "萬",
		7:  "拾",
		8:  "佰",
		9:  "仟",
		10: "亿",
		11: "拾",
		12: "佰",
		13: "仟",
	}
}

func ToRmb(amount float64) string {
	// 金额转为分
	cent_amount := int(math.Round(amount * 100))
	result := ""
	pointer := 0 // 单位指针
	remainder := 0
	last_remainder := 0 // 上一个余数

	for cent_amount > 0 {
		// remainder := int(math.Mod(amount, 10)) // 余数
		remainder = cent_amount % 10   // 余数
		cent_amount = cent_amount / 10 // 缩小10倍
		if *verbosePtr {
			fmt.Printf("remainder: %d, pointer: %d, amount: %d\n",
				remainder, pointer, cent_amount)
		}

		if remainder == 0 {
			if pointer == 2 || pointer == 6 || pointer == 10 {
				// 余数为0, 元、萬、亿单位需要保留
				result = unit_map[pointer] + result
			} else {
				if last_remainder > 0 {
					// 余数为0, 上一个余数也为0, 避免零仟零佰
					result = num_arr[remainder] + result
				}
			}
		} else {
			result = num_arr[remainder] + unit_map[pointer] + result
		}

		last_remainder = remainder
		pointer += 1                      // 从右往左
		if pointer == 2 && "" == result { // 无角、分尾数
			result = "整"
		}

		if *verbosePtr {
			fmt.Println(result)
		}
	}

	return result
}

func main() {
	// ToRmb(10230045.689)
	verbosePtr = flag.Bool("v", false, "verbose output")
	flag.Parse()
	fmt.Println("verbose:", *verbosePtr)

	for {
		inputReader := bufio.NewReader(os.Stdin)
		fmt.Print("Please input(0退出): ")
		input, err := inputReader.ReadString('\n')
		if err == nil {
			trimmedInput := strings.TrimSpace(input)
			if "0" == trimmedInput {
				os.Exit(0)
			}

			amount, err := strconv.ParseFloat(trimmedInput, 64)
			if err != nil {
				fmt.Println("err:", err)
			} else {
				fmt.Printf("%s => %s\n", trimmedInput, ToRmb(amount))
			}
		} else {
			fmt.Println("err:", err)
		}
	}
}
