import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class SupplyStacks {
    private static TreeMap<Character, Deque<Integer>> stacks;
    private static List<Instruction> instructions;

    public record Instruction(int amount, char from, char to) {}

    public static String[] parseStacks(String[] input) {
        return IntStream.iterate(1, i -> i < input[0].length(), i -> i + 4).mapToObj(j ->
                        Arrays.stream(input).mapToInt(s -> s.charAt(j))
                                .collect(StringBuffer::new, StringBuffer::appendCodePoint, StringBuffer::append)
                                .toString())
                .toArray(String[]::new);
    }

    private static void readStacksAndInstructions(String filename) throws IOException {
        String[] parts = Files.readString(Paths.get(filename)).split("\\n{2,}");

        String[] stacks = parseStacks(parts[0].split("\n"));
        SupplyStacks.stacks = new TreeMap<>();
        for (var stack : stacks) {
            var id = stack.charAt(stack.length() - 1);
            var numbers = stack.substring(0, stack.length() - 1).strip()
                    .chars().boxed().collect(Collectors.toCollection(ArrayDeque::new));
            SupplyStacks.stacks.put(id, numbers);
        }

        SupplyStacks.instructions = parts[1].lines()
                .map(line -> {
                    var tokens = line.split(" ");
                    var amount = Integer.parseInt(tokens[1]);
                    return new Instruction(amount, tokens[3].charAt(0), tokens[5].charAt(0));
                }).toList();
    }

    private static Map<Character, Deque<Integer>> cloneStacks() {
        var copy = new TreeMap<Character, Deque<Integer>>();
        stacks.forEach((key, value) -> copy.put(key, new ArrayDeque<>(value)));
        return copy;
    }

    private static String getTopOfStackMessage(Map<Character, Deque<Integer>> stacks) {
        return stacks.keySet().stream().map(key -> stacks.get(key).getFirst())
                .collect(StringBuffer::new, StringBuffer::appendCodePoint, StringBuffer::append)
                .toString();
    }

    private static String part1(Map<Character, Deque<Integer>> stacks, List<Instruction> instructions) {
        for (var instruction : instructions) {
            var fromQueue = stacks.get(instruction.from());
            var toQueue = stacks.get(instruction.to());
            for (int i = 0; i < instruction.amount(); ++i) {
                toQueue.addFirst(fromQueue.pop());
            }
        }
        return getTopOfStackMessage(stacks);
    }

    private static String part2(Map<Character, Deque<Integer>> stacks, List<Instruction> instructions) {
        for (var instruction : instructions) {
            var fromQueue = stacks.get(instruction.from());
            var toQueue = stacks.get(instruction.to());
            var batch = new ArrayDeque<Integer>();
            for (int i = 0; i < instruction.amount(); ++i) {
                batch.addLast(fromQueue.pop());
            }
            for (int i = 0; i < instruction.amount(); ++i) {
                toQueue.addFirst(batch.removeLast());
            }
        }
        return getTopOfStackMessage(stacks);
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            readStacksAndInstructions(filename);

            System.out.printf("Part 1: %s%n", part1(cloneStacks(), instructions));
            System.out.printf("Part 2: %s%n", part2(cloneStacks(), instructions));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
