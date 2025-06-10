class Solution {
    public int maxDifference(String s) {
        int arr[] = new int[26]; 
        int even = 1000, odd = 0;
        for(char i : s.toCharArray()){
            arr[i - 'a']++;
        }
        for(int i : arr){
            if(i%2 == 0 && i != 0)
                even = Math.min(even, i);
            else    
                odd = Math.max(odd, i); 
        }
        return odd - even;
    }
}