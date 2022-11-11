# numeron

an inefficient spaghetti-filled experimental floating-point calculator

evaluation is done by replacing parts of a working-copy string until it turns into a literal.
each step
  1. finds parenthesised sections and evaluates them if present
  2. finds the left-most instance of the lowest-precedence operator present in the working copy
  3. if the operator is not found, tries to parse the working copy as a literal — otherwise fails
  4. if it is found, splits the entire working copy at the operator, and evaluates each side recursively


## todo
- [ ] separate evaluation and parsing
- [ ] rethink operator structure
- [ ] proper juxtaposition instead of stupid hack
- [ ] juxtaposition ambiguity detection