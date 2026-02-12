# Things to do

## Reactivity

- Reverse broadcasters which gather data from multiple sources.
    - Child hints.
    - Marking a window as dirty (preventing it from closing).
- Dynamic arity containers.

## Animation

- Animatable UI.
  - Lerp for UI: the ability to interpolate between two compatible blueprints.
    - If `A: Lerp` and `B: Lerp` you can assume `(A, B): Lerp` and similarly for other product types;
    - If `A: Fade` you can assume `Option<A>: Lerp`;
    - If `A: Lerp`, then `Either<A, A>: Lerp`, similarly for other sum types;
    - Even if `A: ?Lerp, B: ?Lerp`, an adapter `FadeBetween<A, B>` can animate between them;

## Layout

- Implement focus.
    - Integrate with access-kit.

## Graphics

## Stability

- Add tests. Not sure what I'll be testing, but tests need to be here.