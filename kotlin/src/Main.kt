import java.io.File
import java.time.Instant
import kotlin.time.measureTime

fun main() {
    val days: List<IDay> = listOf(Day1());

    days.forEachIndexed{i, day ->
        val dayNum = i + 1
        val input = File("../inputs/$dayNum.txt").readText()
        val timeTakenA = measureTime {
            print("${dayNum}a: ${day.part1(input)}")
        }
        println(", time: $timeTakenA")
        val timeTakenB = measureTime {
            print("${dayNum}b: ${day.part2(input)}")
        }
        println(", time: $timeTakenB")
    }
}