# flag
k : 原来的h将最头的a & b改为 ~a ^ b

# 语法
*~ : DualOut
<* : 向左移动一格
D : D2
A : 全部通过AB生成
N : 输出加INV
I : 本身就是个INV
L{} : 基于何种逻辑运算
E : 异或直接输出 S
O{C} / O{Q} / O{CQ} : 该C或Q会作为最终异或的输出。如有重复则error
M : cell使用镜像逻辑

# 镜像逻辑规则
对于合成G和H，非最后一项的输入是否镜像取决于cell是否M。最后一项是否镜像取决于输出wire是否是m
对于合成P和Q，所有输入都取决于cell是否M，且输出一定也带m

# Special

## VDDH_SAME_VT

[VDH] : power 使用VDDH

