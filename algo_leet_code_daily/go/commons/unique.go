package commons

//O(n)
func toUnique(k []int) []int {
	l := []int{}
	for _, s := range k {
		if !contains(l, s) {
			l = append(l, s)
		}
	}
	return l
}
