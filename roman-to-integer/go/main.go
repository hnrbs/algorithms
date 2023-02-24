package main

func symbolToInt(symbol string) int {
  switch symbol {
  default: return 0 
  case "I", "i": return 1
  case "V", "v": return 5
  case "X", "x": return 10
  case "L", "l": return 50
  case "C", "c": return 100
  case "D", "d": return 500
  case "M", "m": return 1000
  }
}

func romanToInt(symbols string) int {
  total := 0

  for index, char := range symbols {
    current_value := symbolToInt(string(char))
   
    if index+1 >= len(symbols) {
      total += current_value 

      continue
    }

    next_value := symbolToInt(string(symbols[index+1]))

    if current_value < next_value {
      total -= current_value
      continue
    }

    total += current_value
  }

  return total
}

func main() {
  result := romanToInt("MCMXCIV")

  println(result)
}

