function validatePNum(sPNum)
{
  var numbers = sPNum.match(/^(\d)(\d)(\d)(\d)(\d)(\d)(\d)(\d)(\d)(\d)(\d)(\d)$/);
  var checkSum = 0;

  var d = new Date();
  if (!isDate(sPNum.substring(0,4),sPNum.substring(4,6),sPNum.substring(6,8))) {
    console.log("Datumet " + sPNum.substring(0,8) + " är inte korrekt.");
    return false;
  }

  if (numbers == null) { return false; }

  var n;
  for (var i = 3; i <= 12; i++)
  {
    n=parseInt(numbers[i]);
    if (i % 2 == 0) {
      checkSum+=n;
    } else {
      checkSum+=(n*2)%9+Math.floor(n/9)*9
    }
  }

  if (checkSum%10==0) { return true;}
  return false;
}

function getYear(y) { return (y < 1000) ? y + 1900 : y; }

function isDate(year, month, day)
{
  month = month - 1;
  var tmpDate = new Date(year,month,day);
  if ( (getYear(tmpDate.getYear()) == year) &&
  (month == tmpDate.getMonth()) &&
  (day == tmpDate.getDate()) )
    return true;
  else
    return false;
}

console.log(validatePNum("199509155678"));