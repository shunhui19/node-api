// target:
// 1.every key has own rate limit, eg:
//     key1: 10/qps
//     key2: 100/qps
// 2.load the rate limit of every key from config when server start
// 3.when the rate limit of a key is updated in the backgroud,
//   the service can be refreshed in time.
// 4.every key has own total number of request within a period of time. eg:
//   period time: a month / a week

