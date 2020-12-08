package main

import (
    "bufio"
    "fmt"
    "os"
    "strings"
    "strconv"
    //"github.com/davecgh/go-spew/spew"
)

type BagPair struct {
    name string
    amount int
}

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func stringInSlice(a string, list []BagPair) bool {
    for _, b := range list {
        if b.name == a {
            return true
        }
    }
    return false
}

func stringInSliceS(a string, list []string) bool {
    for _, b := range list {
        if b == a {
            return true
        }
    }
    return false
}

func countContained(name string, mapping map[string][]BagPair, depth int) int {
    sum := 0
    for _, p := range (mapping[name]) {
        partial := countContained(p.name, mapping, depth + 1)
        fmt.Println(strings.Repeat(" ", depth), p.amount, p.name, partial)
        sum += p.amount + p.amount * partial
    }
    return sum
}

func main() {
    file, err := os.Open("./input.txt")
    check(err)

    bagMapping := make(map[string][]BagPair)
    
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := strings.TrimSuffix(scanner.Text(), "\n")
        bag_name := strings.Split(line, " contain")[0]
        contents := strings.Split(strings.TrimSuffix(strings.Split(line, "contain ")[1], "."), ", ")
        // fmt.Println(bag_name)
        for _, c := range contents {
            if c == "no other bags" {
                bagMapping[bag_name] = make([]BagPair, 0)
            } else {
                num, _ := strconv.Atoi(c[0:1])
                b_name := c[2:]
                if b_name[len(b_name)-1] != 's' {
                    b_name = b_name + "s" // Aboid singular vs plural issues
                }
                bagMapping[bag_name] = append(bagMapping[bag_name], BagPair{b_name, num})
            }
            // fmt.Println("  ", c)
        }
    }

    shiny_containers := make([]string, 0)
    queue := make([]string, 0)
    for k, v := range bagMapping {
        if stringInSlice("shiny gold bags", v) {
            queue = append(queue, k)
            shiny_containers = append(shiny_containers, k)
        }
    }

    for len(queue) > 0 {
        q := queue[0]
        queue = queue[1:]

        for k, v := range bagMapping {
            if stringInSlice(q, v) {
                queue = append(queue, k)
                if !stringInSliceS(k, shiny_containers) {
                    shiny_containers = append(shiny_containers, k)
                }
            }
        }
    }

    fmt.Println("Done")
    fmt.Println(len(queue))
    fmt.Println(len(bagMapping))
    fmt.Println(len(shiny_containers))

    fmt.Println(countContained("shiny gold bags", bagMapping, 0))
}
