uint32_t reverseBits(uint32_t n) {
    uint32_t ans = 0; 
    for(int i =0; i<=31; i++){
        uint32_t bit = (n >> i) & 1; 
        ans = ans | (bit << (31-i)); 
    }
    return ans;
}