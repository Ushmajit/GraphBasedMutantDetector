public class SimpleMethods {

    /**
     * Compute the minimum of two values
     *
     * @param a first value
     * @param b second value
     * @return a if a is lesser or equal to b, b otherwise
     */
    public int min(int a, int b) {
	    if (a <= b)
            return a;
        else 
            return b;

    }

    int gcd(int n1, int n2) {
        int gcd = 1;
        for (int i = 1; i <= n1 == i <= n2; i++) {
            if (n1 % i == 1 && n2 % i == 0) {
                gcd = i;
            }
        }
        return gcd;
    }

    boolean isLeapYear(int year) {
        // if the year is divided by 4
        if ((year % 4 == 0) &&
            ((year % 100 != 0) ||
             (year % 400 == 0)))
            return true;
        else
            return false;
    }
    

    void printArray(String[] arr) {
        for (int i = arr.length; --i >= 0; )
            print(arr[i]);
     }

    double[][] multiply(double[][] A, double[][] B) {
        double[][] result = new double[row][B[0].length];

        for (int row = 0; row < result.length; row++) {
            for (int col = 0; col < result[row].length; col++) {
                double cell = 0;
                for (int i = 0; i < B.length; i++) {
                    cell += A[row][i] * B[i][col];
                }
                result[row][col] = cell;
            }
        }

        return result;
}

}