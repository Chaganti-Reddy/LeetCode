class TimeMap:

    def __init__(self):
        self.t = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key not in self.t:
            self.t[key] = []
        self.t[key].append([timestamp, value])

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.t:
            return ""
        
        values = self.t[key]
        l, r = 0, len(values) - 1
        res = ""
        
        while l <= r:
            mid = (l + r) // 2
            if values[mid][0] == timestamp:
                return values[mid][1]
            elif values[mid][0] < timestamp:
                res = values[mid][1]  # potential candidate
                l = mid + 1
            else:
                r = mid - 1
        
        return res
