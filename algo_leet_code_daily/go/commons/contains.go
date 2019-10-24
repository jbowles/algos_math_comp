package commons

func contains(c []int, in int) bool {
	for i := range c {
		if c[i] == in {
			return true
		}
	}
	return false
}
