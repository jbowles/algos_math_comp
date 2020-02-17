package main

import "fmt"

/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.

*/

func main() {
	first := []int{2, 4, 3}
	second := []int{5, 6, 4}
	output := []int{}
	for i := 0; i < len(first); i++ {
		fmt.Printf("%d %d\n", first[i], second[i])
		l1 := &ListNode{Val: first[i]}
		l2 := &ListNode{Val: second[i]}
		ln := addTwoNumbers(l1, l2)
		output = append(output, ln.Val)
	}
	fmt.Printf("%v\n", output)

	fmt.Println(addTwoNumbers(&ListNode{Val: 2}, &ListNode{Val: 5}))
	fmt.Println(addTwoNumbers(&ListNode{Val: 4}, &ListNode{Val: 6}))
	fmt.Println(addTwoNumbers(&ListNode{Val: 3}, &ListNode{Val: 4}))
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	tail := &ListNode{0, nil}
	head := tail
	var num int
	for l1 != nil || l2 != nil {
		num /= 10
		if l1 != nil {
			num += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			num += l2.Val
			l2 = l2.Next
		}
		head.Next = &ListNode{Val: num % 10}
		head = head.Next
	}
	if num/10 == 1 {
		head.Next = &ListNode{Val: 1}
	}
	return tail.Next
}
