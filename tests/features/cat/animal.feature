Feature: Animal feature

  Scenario: If we feed a hungry cat it will no longer be hungry
    Given a hungry cat
    When I feed the cat
    Then the cat is not hungry

  Scenario: if we feed a satiated cat it still is not hungry
    Given a satiated cat
    When I feed the cat
    Then the cat vomits
    Then the cat is not hungry

  Scenario: if we feed a starved cat it still is not hungry
    Given a starved cat
    When I feed the cat
    Then the cat is not hungry
