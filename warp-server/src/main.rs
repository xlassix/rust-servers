use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex as TokioMutex;
use warp::{Filter, Reply};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Employee {
    name: String,
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: u32,
}
type EmployeeHashMap = HashMap<u32, Employee>;

type EmployeeMap = Arc<TokioMutex<EmployeeHashMap>>;

async fn fetch_employee_by_id(id: u32, employees: EmployeeMap) -> Option<Employee> {
    let locked_employees = employees.lock().await;
    return locked_employees.get(&id).cloned();
}

// handle API request
async fn get_employee_name_by_id_handler(
    id: u32,
    employees: EmployeeMap,
) -> Result<impl Reply, warp::Rejection> {
    match fetch_employee_by_id(id, employees).await {
        Some(employee) => Ok(warp::reply::json(&Employee {
            id: employee.id,
            name: employee.name,
        })),
        None => Ok(warp::reply::json(&format!(
            "An Employee with ID:{} was not found",
            id
        ))),
    }
}

// async fn save_employee(
//     new_emp: Employee,
//     employees: &mut EmployeeHashMap,
// ) -> Result<impl Reply, warp::Rejection> {
//     let c = employees.borrow_mut();
//     // let mut locked_employees = (employees.clone().write().await).clone();
//     let id: u32 = employees.len() as u32;
//     employees.insert(
//         id,
//         Employee {
//             name: new_emp.name,
//             id: id,
//         },
//     );

//     return Ok(warp::reply::json(&Response { id: 1 }));
// }

#[tokio::main]
async fn main() {
    const PORT: u16 = 4040;
    let employees: EmployeeMap = Arc::new(TokioMutex::new(HashMap::new()));

    {
        let mut locked_employee = employees.lock().await;
        locked_employee.insert(
            1,
            Employee {
                name: "John Doe".to_string(),
                id: 1,
            },
        );
        locked_employee.insert(
            2,
            Employee {
                name: "Mary Jane".to_string(),
                id: 2,
            },
        );
        locked_employee.insert(
            3,
            Employee {
                name: "Peter Parker".to_string(),
                id: 3,
            },
        );
    }

    let get_all_employees = warp::path!("api" / "employee").and(warp::get()).map({
        let locked_employees = (employees.clone().lock().await).clone();
        let all_employees: Vec<Employee> = locked_employees.into_iter().map(|(_, v)| v).collect();
        move || warp::reply::json(&all_employees)
    });

    let get_employee = warp::path!("api" / "employee" / u32)
        .and(warp::get())
        .and(warp::any().map(move || employees.clone()))
        .and_then(get_employee_name_by_id_handler);

    // let save_employees = warp::path!("api" / "employee")
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .map(move |new_emp: Employee| {
    //         // let mut locked_employees = employees.blocking_lock();
    //         // let all_employees: Vec<Employee> = locked_employees.into_iter().map(|(_, v)| v).collect();
    //         employees.l().insert(4, new_emp);
    //         // println!("{:?}", new_emp);
    //         return warp::reply::json(&Response { id: 1 });
    //     });

    warp::serve(get_all_employees.or(get_employee))
        .run(([127, 0, 0, 1], PORT))
        .await;
}
