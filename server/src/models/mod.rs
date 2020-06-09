mod entry;
mod food;
mod meal;
mod meal_type;
mod user;

use diesel::dsl::Select;
use diesel::query_source::Table;
pub use entry::Entry;
pub use food::Food;
pub use meal::Meal;
pub use meal_type::{MealType, NewMealType};
pub use user::{NewUser, User};

pub type Session = NewUser;
type All<T: Table> = Select<T, T::AllColumns>;
