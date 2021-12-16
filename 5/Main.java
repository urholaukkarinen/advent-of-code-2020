import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.OptionalInt;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) throws IOException {
        var boardingPasses = Files.readAllLines(Path.of("input.txt"));
        var seatIds = boardingPasses.stream()
                .map(Main::toSeatId)
                .sorted()
                .collect(Collectors.toList());

        var partOne = seatIds.get(seatIds.size() - 1);
        var partTwo = findMissing(seatIds).orElseThrow();

        System.out.println(partOne);
        System.out.println(partTwo);
    }

    private static OptionalInt findMissing(List<Integer> seatIds) {
        for (var i = 0; i < seatIds.size() - 1; i++) {
            var a = seatIds.get(i);
            var b = seatIds.get(i + 1);
            if (a != b - 1) {
                return OptionalInt.of(a + 1);
            }
        }
        return OptionalInt.empty();
    }

    private static int toSeatId(String boardingPass) {
        var data = boardingPass.getBytes();

        var min = 0;
        var max = 127;

        var row = 0;
        var col = 0;

        for (var i = 0; i < data.length; i++) {
            switch (data[i]) {
                case 'F':
                case 'L':
                    max -= (max - min) / 2 + 1;
                    break;
                case 'B':
                case 'R':
                    min += (max - min) / 2 + 1;
                    break;
                default:
            }

            if (i == 6) {
                row = max;
                min = 0;
                max = 7;
            }
        }
        col = max;

        return row * 8 + col;
    }
}