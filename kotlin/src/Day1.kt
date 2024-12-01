import kotlin.math.abs

class Day1 : IDay {
    private fun getNumbers(line: String): Pair<Int, Int> {
        val numbers = line.split(Regex(" +")).map { it.toInt() }
        return numbers[0] to numbers[1]
    }

    override fun part1(input: String) : String {
        val items = input
            .lines()
            .map { line -> getNumbers(line) }

        val firstList = items.map { it.first }.sorted()
        val secondList = items.map { it.second }.sorted()

        return firstList.zip(secondList).sumOf { (first, second) ->
            abs(first - second)
        }.toString()
    }

    override fun part2(input: String) : String {
        val items = input
            .lines()
            .map { line -> getNumbers(line) }

        val firstList = items.map { it.first }
        val countMap = items.map { it.second }
            .fold(mutableMapOf<Int, Int>()) { countMap, value ->
                countMap[value] = (countMap[value] ?: 0) + 1
                countMap
            }

        return firstList.sumOf { value ->
            value * (countMap[value] ?: 0)
        }.toString()
    }
}