Feature: Animal feature

  Scenario: If we feed a hungry cat it will no longer be hungry
    Given a hungry cat
    When I feed the cat
    Then the cat is not hungry
    Then the cat is satiated
    Then the cat is not dead
    Then the cat is alive
    Then the cat is not vomiting

  Scenario: if we feed a satiated cat it will be sick
    Given a satiated cat
    When I feed the cat
    Then the cat is vomiting
    Then the cat is not hungry
    Then the cat is satiated
    Then the cat is not dead
    Then the cat is alive

  Scenario: if we starve a satiated cat he will be ok
    Given a satiated cat
    When I starve the cat
    Then the cat is hungry
    Then the cat is not satiated
    Then the cat is not dead
    Then the cat is alive
    Then the cat is not vomiting

  Scenario: if we insist feeding a sick cat, he dies
    Given a vomiting cat
    When I feed the cat
    Then the cat is dead
    Then the cat is not alive

  Scenario: if we insist starving a cat, it dies
    Given a starving cat
    When I starve the cat
    Then the cat is dead
    Then the cat is not alive
