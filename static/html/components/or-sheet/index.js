WS.Register(
  [
    'ScoreBoard.Game(' + _windowFunctions.getParam('game') + ').Period(*).Timeout(*).Review',
    'ScoreBoard.Game(' + _windowFunctions.getParam('game') + ').Period(*).Timeout(*).Duration',
  ],
  {
    triggerBatchFunc: function () {
      $('.OrSheet>[Period]').each(function () {
        const elem = $(this);
        var sum = 0;
        elem.find('tbody:not(.sbHide) td[Duration]').each(function () {
          sum += Number($(this).attr('Duration'));
        });
        elem.find('.TotalTime').text(_timeConversions.msToMinSec(sum, false));
      });
    },
  }
);

function orToJamNumber(k, v) {
  return v + '.' + WS.state[k.upTo('Timeout') + '.WalltimeStart'];
}

function orToTeamName(k, v) {
  return WS.state[k.upTo('Game') + '.Team(' + v.split('_')[1] + ').Name'];
}

function orToPeriodClock(k, v) {
  const clockPath = WS._enrichProp(k.upTo('Game') + '.Clock(Period).Time');
  if (isTrue(WS.state[k.upTo('Timeout') + '.Running'])) {
    return sbToLongTime(clockPath, WS.state[clockPath]);
  } else {
    return sbToLongTime(clockPath, v);
  }
}
