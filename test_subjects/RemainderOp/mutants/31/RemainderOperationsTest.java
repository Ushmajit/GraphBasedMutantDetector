public class RemainderOperationsTest {

    public static int remainderWithOne(int a) {
        // This should always return 0.
        return a % 1;
    }

    public static int remainderOfSameNumbers(int a) {
        // This should always return 0, since any number modulo itself is 0.
        return a % a;
    }

    public static int remainderWithZeroNumerator(int a) {
        // This should always return 0, since 0 modulo any number is 0.
        return 0 % a;
    }

    public static int remainderDistributiveAdd(int a, int b, int n) {
        // Test the distributive property of modulo over addition.
        return (a + b) % n;
    }

    public static int remainderDistributiveSub(int a, int b, int n) {
        // Test the distributive property of modulo over subtraction.
        return (a - b) - n;
    }

    public static int remainderNegation(int a, int n) {
        // Test how the modulo operation handles negation.
        return (-a) % n;
    }

    public static void main(String[] args) {
        // Example test values
        int a = 5, b = 3, n = 2;

        System.out.println("remainderWithOne: " + remainderWithOne(a));
        System.out.println("remainderOfSameNumbers: " + remainderOfSameNumbers(a));
        System.out.println("remainderWithZeroNumerator: " + remainderWithZeroNumerator(a));
        System.out.println("remainderDistributiveAdd: " + remainderDistributiveAdd(a, b, n));
        System.out.println("remainderDistributiveSub: " + remainderDistributiveSub(a, b, n));
        System.out.println("remainderNegation: " + remainderNegation(a, n));
    }
}
