function solution(cards1, cards2, goal) {
  let check_cards1 = false;
  let cards1_order = 0;
  let check_cards2 = false;
  let cards2_order = 0;

  if (cards1.length == goal.length) {
    let check = false;
    for (let k = 0; k < goal.length; k++) {
      if (cards1[k] == goal[k]) {
        check = true;
      }
    }
    if (check) return "Yes";
  } else if (cards2.length == goal.length) {
    let check2 = false;
    for (let l = 0; l < goal.length; l++) {
      if (cards2[l] == goal[l]) {
        check2 = true;
      }
    }
    if (check2) return "Yes";
  }
  for (let i = 0; i < cards1.length; i++) {
    if (
      goal.indexOf(cards1[i]) >= cards1_order &&
      goal.indexOf(cards1[i]) >= 0
    ) {
      check_cards1 = true;
      cards1_order = goal.indexOf(cards1[i]);
    } else {
      if (cards2.length + i + 1 > goal.length) {
        return "Yes";
      }
      return "No";
    }
  }

  if (check_cards1) {
    for (let j = 0; j < cards2.length; j++) {
      if (
        goal.indexOf(cards2[j]) >= cards2_order &&
        goal.indexOf(cards2[j]) >= 0
      ) {
        check_cards2 = true;
        cards2_order = goal.indexOf(cards2[j]);
      } else {
        if (cards1.length + j + 1 > goal.length) {
          return "Yes";
        }
        return "No";
      }
    }
  }

  if (
    check_cards1 &&
    check_cards2 &&
    cards1.length + cards2.length >= goal.length
  )
    return "Yes";
  return "No";
}
