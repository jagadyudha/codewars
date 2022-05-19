// Write a function that when given a URL as a string, parses out just the domain name and returns it as a string. For example:

// * url = "http://github.com/carbonfive/raygun" -> domain name = "github"
// * url = "http://www.zombie-bites.com"         -> domain name = "zombie-bites"
// * url = "https://www.cnet.com"                -> domain name = cnet"

function domainName(url) {
  const isHttp = url.startsWith("http");
  const result = isHttp
    ? url.replace("www.", "").split("/")[2].split(".")[0]
    : url.replace("www.", "").split(".")[0];
  if (result == "") {
    return result;
  } else {
    return result;
  }
}

console.log(domainName("www.google.com"));
console.log(domainName("https://www.google.com"));
console.log(domainName("https://google.com"));
console.log(domainName("google.com"));
console.log(domainName("google.com"));
console.log(domainName("google"));
