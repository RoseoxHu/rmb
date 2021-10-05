/**
 * 人民币大写转换
 */
import java.util.*;
import java.math.BigDecimal;
 
public final class Rmb {
    final static String[] CN_NUMS = {"零","壹","贰","叁","肆","伍","陆","柒","捌","玖"};
    final static String[] CN_UNIT = {"分","角","元","拾","佰","仟","萬","拾","佰","仟","亿","拾","佰","仟"};
    
    final Map<Integer, String> NUM_MAP = new HashMap<>(10); 
    final Map<Integer, String> UNIT_MAP = new HashMap<>(15); 

    
    public static String toRmb(double amount) {
        amount = Math.round(amount * 100); // 金额转为分
        
        String result = "";
        int pointer = 0;        // 单位指针
        int remainder = 0;
        int last_remainder = 0; // 上一个余数
        
        while (amount > 0) {
            remainder = (int)(amount % 10); // 余数 不兼容的类型: 从double转换到int可能会有损失
            amount = (int) (amount / 10);  // 缩小10倍
            System.out.println(String.format("amount: %s, remainder: %s, pointer: %s",
                amount, remainder, pointer));

            if (remainder == 0) {
                if (pointer == 2 || pointer == 6 || pointer == 10) {
                    // 余数为0, 元、萬、亿单位需要保留
                    result = CN_UNIT[pointer] + result;
                } else {
                    if (last_remainder > 0) {
                        // 余数为0, 上一个余数也为0, 避免零仟零佰
                        result = CN_NUMS[remainder] + result;
                    }
                }
            } else {
                result = CN_NUMS[remainder] + CN_UNIT[pointer] + result;
            }
            last_remainder = remainder;
            pointer += 1;    // 从右往左
            if (pointer == 2 && "".equals(result)) {  // 无角、分尾数
                result = "整";
            }
            System.out.println(result);
        }// end while
        
        return result;
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.out.println("Usage: Rmb <金额>");
            System.exit(0);
        }
        
		try {
			// double amount = Math.round(new Double(args[1]) * 100) / 100.0;
			BigDecimal amount = new BigDecimal(args[0]);
			amount.setScale(2, BigDecimal.ROUND_HALF_UP);
			amount = amount.multiply(new BigDecimal("100")).divide(new BigDecimal("100.0"));
			// if (amount >= 1000000000000) {  错误: 过大的整数: 1000000000000
			if (amount.compareTo(new BigDecimal("1000000000000")) >= 0) {
				System.out.println("Usage: Rmb 金额超出范围[0, 1000000000000)");
				System.exit(0);
			}
			
			toRmb(amount.doubleValue());
		} catch (Exception e) {
			System.out.println(e);
			System.out.println("Usage: Rmb <金额>");
			System.exit(0);
		}
		
    }
}